import { describe, expect, it } from "vitest";
import { highlightText } from "../highlight";

describe("highlightText", () => {
  it("returns full text when query is empty", () => {
    const result = highlightText("Hello World", "");
    expect(result).toEqual([{ text: "Hello World", highlight: false }]);
  });

  it("highlights matches in a case-insensitive way", () => {
    const result = highlightText("Hello World", "world");
    expect(result).toEqual([
      { text: "Hello ", highlight: false },
      { text: "World", highlight: true },
    ]);
  });

  it("handles multiple occurrences with regex characters", () => {
    const result = highlightText("a+b a+b a+b", "a+b");
    expect(result).toEqual([
      { text: "a+b", highlight: true },
      { text: " ", highlight: false },
      { text: "a+b", highlight: true },
      { text: " ", highlight: false },
      { text: "a+b", highlight: true },
    ]);
  });
});
