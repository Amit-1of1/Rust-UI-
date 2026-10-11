export type Priority = 'Low' | 'Medium' | 'High';
export type Status = 'Todo' | 'Doing' | 'Done';

export interface Task {
  id: number;
  title: string;
  assignee: string;
  priority: Priority;
  status: Status;
  estimate: number;
}

const ADJ = ['Quick', 'Lazy', 'Bright', 'Quiet', 'Brave', 'Calm', 'Eager', 'Fancy', 'Gentle', 'Happy'];
const NOUN = ['report', 'review', 'deploy', 'design', 'fix', 'test', 'sync', 'plan', 'audit', 'launch'];
export const NAMES = ['Ada', 'Grace', 'Linus', 'Margaret', 'Alan'];
export const PRIORITIES: Priority[] = ['Low', 'Medium', 'High'];
export const STATUSES: Status[] = ['Todo', 'Doing', 'Done', 'Done'];

export function makeTask(i: number): Task {
  return {
    id: i + 1,
    title: `${ADJ[i % 10]} ${NOUN[(i * 7) % 10]} #${i + 1}`,
    assignee: NAMES[i % 5],
    priority: PRIORITIES[i % 3],
    status: STATUSES[i % 4],
    estimate: ((i * 37) % 13) + 1,
  };
}

export function makeTasks(start: number, count: number): Task[] {
  const out = new Array<Task>(count);
  for (let k = 0; k < count; k++) out[k] = makeTask(start + k);
  return out;
}

export function rowCountFromUrl(): number {
  const n = Number.parseInt(new URLSearchParams(location.search).get('rows') ?? '', 10);
  return Number.isFinite(n) && n >= 0 ? n : 200;
}
