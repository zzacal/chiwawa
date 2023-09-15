import type { Action } from "../pages/library/types";

export type Tab = ActionTab;

export type ActionTab = {
  id: string;
  label: string,
  type: "request",
  methods: string[],
  action: Action
}
