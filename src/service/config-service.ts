import { invoke } from "@tauri-apps/api";
import type { Config } from "../types";

export async function getConfig(): Promise<Config> {
  return await invoke("initialize");
}
