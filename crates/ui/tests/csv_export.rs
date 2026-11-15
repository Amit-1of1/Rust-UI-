//! `table_csv` on random tables, read back by an independent RFC 4180 parser: the file holds
//! exactly the header and the requested rows of the shown columns, every cell survives the
//! round trip (apart from the formula guard), and no cell a spreadsheet would run as a
//! formula gets through. The guard is the security claim: a cell starting with `=`, `+`, `-`,
//! `@`, tab or carriage return is prefixed with `'`, unless it is a finite number in a numeric
//! column (OWASP "CSV injection").

use proptest::prelude::*;
use ui::components::data_table::*;

/// RFC 4180, written for this test: records end in CRLF, a quoted field may hold commas,
/// quotes (doubled) and line breaks.
fn parse(csv: &str) -> Vec<Vec<String>> {
    let mut records = Vec::new();
    let mut record = Vec::new();
    let mut field = String::new();
    let mut chars = csv.chars().peekable();
    let mut quoted = false;
    let mut at_start = true;
    while let Some(c) = chars.next() {
        if quoted {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    field.push('"');
                } else {
                    quoted = false;
                }
            } else {
                field.push(c);
            }
            continue;
        }
        match c {
            '"' if at_start => {
                quoted = true;
                at_start = false;
            }
            ',' => {
                record.push(std::mem::take(&mut field));
                at_start = true;
            }
            '\r' if chars.peek() == Some(&'\n') => {
                chars.next();
                record.push(std::mem::take(&mut field));
                records.push(std::mem::take(&mut record));
                at_start = true;
            }
            _ => {
                assert!(c != '"', "a bare quote inside an unquoted field: {csv:?}");
                field.push(c);
                at_start = false;
            }
        }
    }
    assert!(!quoted, "an unterminated quoted field: {csv:?}");
    assert!(
        field.is_empty() && record.is_empty(),
        "the file doesn't end with CRLF: {csv:?}"
    );
    records
}

/// What a spreadsheet would treat as a number: only digits, one sign, a point, an exponent,
/// and the separators and symbols `table_view` ignores. Finite.
fn plain_number(cell: &str) -> bool {
    let cleaned: String = cell
        .chars()
        .filter(|c| !matches!(c, '$' | '€' | '£' | ',' | '%' | ' '))
        .collect();
    !cleaned.is_empty()
        && cleaned
            .chars()
            .all(|c| c.is_ascii_digit() || matches!(c, '+' | '-' | '.' | 'e' | 'E'))
        && cleaned.parse::<f64>().is_ok_and(f64::is_finite)
}

fn risky(cell: &str) -> bool {
    cell.starts_with(['=', '+', '-', '@', '\t', '\r'])
}

fn cell() -> impl Strategy<Value = String> {
    // Small pieces that hit every branch: formula starts, quoting, numbers, words that Rust
    // parses as numbers ("inf", "NaN"), spaces at either end.
    let piece = prop::sample::select(vec![
        "=",
        "+",
        "-",
        "@",
        "\t",
        "\r",
        "\n",
        "\r\n",
        ",",
        "\"",
        "'",
        " ",
        "1",
        "0.5",
        "e3",
        "$",
        "%",
        "inf",
        "NaN",
        "a",
        "SUM(A1)",
        "cmd|' /C calc'!A0",
        "é",
    ]);
    prop::collection::vec(piece, 0..5).prop_map(|p| p.concat())
}

fn table() -> impl Strategy<Value = (Vec<DataColumn>, Vec<Vec<String>>, Vec<usize>)> {
    (1usize..5).prop_flat_map(|width| {
        let columns = prop::collection::vec((cell(), any::<bool>(), any::<bool>()), width)
            .prop_map(|cols| {
                cols.into_iter()
                    .map(|(header, numeric, hidden)| {
                        let mut c = DataColumn::new(header);
                        c.numeric = numeric;
                        c.hidden = hidden;
                        c
                    })
                    .collect::<Vec<_>>()
            });
        // Rows may be short (missing cells export as empty) and the order may name rows
        // that don't exist (skipped).
        let rows = prop::collection::vec(prop::collection::vec(cell(), 0..=width), 0..5);
        let order = prop::collection::vec(0usize..7, 0..7);
        (columns, rows, order)
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(4096))]

    #[test]
    fn csv_round_trips_and_never_exports_a_formula((columns, rows, order) in table()) {
        let csv = table_csv(&columns, &rows, &order);
        let records = parse(&csv);
        let shown: Vec<usize> = (0..columns.len()).filter(|&c| !columns[c].hidden).collect();
        let wanted: Vec<usize> = order.iter().copied().filter(|&i| i < rows.len()).collect();
        prop_assert_eq!(records.len(), 1 + wanted.len(), "{:?}", csv);

        let check = |field: &str, original: &str, numeric: bool| -> Result<(), TestCaseError> {
            // The cell comes back as it was, or with the guard in front.
            let guarded = field != original;
            if guarded {
                prop_assert_eq!(field, format!("'{original}"));
                prop_assert!(risky(original), "guarded a harmless cell {:?}", original);
            }
            // The guard's promise: nothing a spreadsheet would evaluate.
            if risky(field) {
                prop_assert!(numeric && plain_number(field), "formula exported: {:?}", field);
            }
            Ok(())
        };

        if shown.is_empty() {
            // Every column hidden: a line per record with nothing on it, which a reader sees
            // as one empty field.
            prop_assert!(records.iter().all(|r| r == &[String::new()]), "{:?}", csv);
            return Ok(());
        }
        let header = &records[0];
        prop_assert_eq!(header.len(), shown.len());
        for (field, &c) in header.iter().zip(&shown) {
            check(field, &columns[c].header, false)?;
        }
        for (record, &i) in records[1..].iter().zip(&wanted) {
            prop_assert_eq!(record.len(), shown.len());
            for (field, &c) in record.iter().zip(&shown) {
                let original = rows[i].get(c).map(String::as_str).unwrap_or("");
                check(field, original, columns[c].numeric)?;
            }
        }
    }
}

#[test]
fn words_that_parse_as_floats_are_still_guarded() {
    let mut col = DataColumn::new("Amount");
    col.numeric = true;
    let rows = vec![
        vec!["-inf".to_string()],
        vec!["-NaN".to_string()],
        vec!["-1,200".to_string()],
        vec!["=1+1".to_string()],
    ];
    let csv = table_csv(&[col], &rows, &[0, 1, 2, 3]);
    assert_eq!(csv, "Amount\r\n'-inf\r\n'-NaN\r\n\"-1,200\"\r\n'=1+1\r\n");
}
