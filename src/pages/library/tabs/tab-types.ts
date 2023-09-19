import type { EnabledKvp } from "../types";

export type ActionTabContent = {
  label: string,
  isEditable: boolean,
} & (HeadersTabContent | BodyTabContent | ParametersTabContent);

export type HeadersTabContent = {
  type: "headers",
  label: "Headers",
  content: EnabledKvp<string, string>[],
  onUpdate?: (val: EnabledKvp<string, string>[]) => void
}

export type BodyTabContent = {
  type: "body",
  label: "Body"
  content: string,
  onUpdate?: (val: string) => void
}

export type ParametersTabContent = {
  type: "params",
  label: "Parameters"
  content: {
    query: EnabledKvp<string, string>[];
    path: EnabledKvp<string, string>[];
  }
  onUpdate?: (val: {
    query: EnabledKvp<string, string>[];
    path: EnabledKvp<string, string>[];
  }) => void
}
