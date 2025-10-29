import type { ClipboardItem } from "$lib/types";

export type DateGroup = "today" | "yesterday" | "this-week" | "this-month" | "older";

export interface GroupedItems {
  group: DateGroup;
  label: string;
  items: ClipboardItem[];
}

export function getDateGroup(date: Date): DateGroup {
  const now = new Date();
  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
  const itemDate = new Date(date.getFullYear(), date.getMonth(), date.getDate());

  const diffTime = today.getTime() - itemDate.getTime();
  const diffDays = Math.floor(diffTime / (1000 * 60 * 60 * 24));

  if (diffDays === 0) return "today";
  if (diffDays === 1) return "yesterday";
  if (diffDays <= 7) return "this-week";
  if (diffDays <= 30) return "this-month";
  return "older";
}

export function getDateGroupLabel(group: DateGroup): string {
  const labels: Record<DateGroup, string> = {
    today: "Today",
    yesterday: "Yesterday",
    "this-week": "This Week",
    "this-month": "This Month",
    older: "Older",
  };
  return labels[group];
}

export function groupItemsByDate(items: ClipboardItem[]): GroupedItems[] {
  // Group items by date
  const groups = new Map<DateGroup, ClipboardItem[]>();

  for (const item of items) {
    const group = getDateGroup(item.createdAt);
    if (!groups.has(group)) {
      groups.set(group, []);
    }
    groups.get(group)!.push(item);
  }

  // Convert to array and sort by group order
  const groupOrder: DateGroup[] = ["today", "yesterday", "this-week", "this-month", "older"];
  const result: GroupedItems[] = [];

  for (const group of groupOrder) {
    const items = groups.get(group);
    if (items && items.length > 0) {
      result.push({
        group,
        label: getDateGroupLabel(group),
        items,
      });
    }
  }

  return result;
}

export function formatTimestamp(date: Date): string {
  const now = new Date();
  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
  const itemDate = new Date(date.getFullYear(), date.getMonth(), date.getDate());

  const diffDays = Math.floor((today.getTime() - itemDate.getTime()) / (1000 * 60 * 60 * 24));

  const timeStr = date.toLocaleTimeString("en-US", {
    hour: "numeric",
    minute: "2-digit",
    second: "2-digit",
    hour12: false,
  });

  if (diffDays === 0) {
    return `today at ${timeStr}`;
  } else if (diffDays === 1) {
    return `yesterday at ${timeStr}`;
  } else if (diffDays <= 7) {
    const dayName = date.toLocaleDateString("en-US", { weekday: "long" });
    return `${dayName} at ${timeStr}`;
  } else {
    const dateStr = date.toLocaleDateString("en-US", {
      month: "short",
      day: "numeric",
      year: date.getFullYear() !== now.getFullYear() ? "numeric" : undefined,
    });
    return `${dateStr} at ${timeStr}`;
  }
}
