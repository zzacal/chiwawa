import { invoke } from "@tauri-apps/api";
import type { Config } from "../types";
import type { LibraryNode } from "../pages/library/types";

export async function getConfig(): Promise<Config> {
  return await invoke("initialize");
}

export async function getLibrary(): Promise<LibraryNode> {
  const getAct = { id:"abcde", name: "Some Get", method: "GET", url: "https://httpbin.org/get"};
  const postAct = { id:"abcdf", name: "Some Post", method: "POST", url: "https://httpbin.org/post"};
  const putAct = { id:"abcdg", name: "Some Put", method: "PUT", url: "https://httpbin.org/put"};
  const deleteAct = { id:"abcdc", name: "Some Delete", method: "DELETE", url: "https://httpbin.org/delete"};
  const defaultLib = { id:"abcdx", name: "Library", actions: [getAct, postAct, putAct, deleteAct] };

  return {...defaultLib, children: [{...defaultLib, name: "Child"}, {...defaultLib, name: "Child2"}]};
}
