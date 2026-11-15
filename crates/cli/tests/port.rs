//! `atoll port` end to end: a file in, code and report out.
use std::process::Command;

#[test]
fn ports_a_file_and_reports_what_is_left() {
    let tmp = tempfile::tempdir().unwrap();
    let file = tmp.path().join("settings.tsx");
    std::fs::write(
        &file,
        r#"import { Switch } from "@/components/ui/switch"
import { Label } from "@/components/ui/label"
import { Separator } from "@/components/ui/separator"
export default function Settings() {
  return (
    <div className="space-y-4 p-6">
      <h2 className="text-lg font-semibold">Notifications</h2>
      <Separator />
      <div className="flex items-center justify-between">
        <Label htmlFor="email-alerts">Email alerts</Label>
        <Switch id="email-alerts" checked={enabled} onCheckedChange={setEnabled} />
      </div>
    </div>
  )
}"#,
    )
    .unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_atoll"))
        .arg("port")
        .arg(&file)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let text = String::from_utf8(out.stdout).unwrap();
    assert!(text.contains("atoll add label separator switch"), "{text}");
    assert!(
        text.contains("Separator {  }") || text.contains("Separator { }"),
        "{text}"
    );
    assert!(text.contains("checked: enabled"), "{text}");
    assert!(text.contains("handler onCheckedChange"), "{text}");
    assert!(text.contains("justify-content: space-between"), "{text}");

    let json = Command::new(env!("CARGO_BIN_EXE_atoll"))
        .args(["port", "--json"])
        .arg(&file)
        .output()
        .unwrap();
    let v: serde_json::Value = serde_json::from_slice(&json.stdout).unwrap();
    assert_eq!(v["trees"], 1);
    assert_eq!(
        v["add"],
        serde_json::json!(["label", "separator", "switch"])
    );
}

#[test]
fn a_file_without_jsx_fails() {
    let tmp = tempfile::tempdir().unwrap();
    let file = tmp.path().join("util.ts");
    std::fs::write(
        &file,
        "export const add = (a: number, b: number) => a + b;\n",
    )
    .unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_atoll"))
        .arg("port")
        .arg(&file)
        .output()
        .unwrap();
    assert!(!out.status.success());
}
