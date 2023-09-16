import { invoke } from "@tauri-apps/api";
import type { Config } from "../types";
import type { LibraryNode } from "../pages/library/types";

export async function getConfig(): Promise<Config> {
  return await invoke("initialize");
}

export async function getLibrary(): Promise<LibraryNode[]> {
  return [...lib];
}

export async function putAction(libId: string | null = null): Promise<LibraryNode[]> {
  if(!libId) {
    return [makeLib(), ...lib]
  }
  return lib
}

/*
* temporary stubs 
*/
function makeLib(): LibraryNode {
  return {
    id: "l"+Date.now().toString(),
    name: "newlib",
    actions: [
      {
        id: "a"+Date.now().toString(),
        name: "newact",
        method: "GET",
        url: "https://chiwawa.com",

      }
    ]
  }
}

const getAct = { id:"abcde", name: "Some Get", method: "GET", url: "https://httpbin.org/get"};
const postAct = { id:"abcdf", name: "Some Post", method: "POST", url: "https://httpbin.org/post"};
const putAct = { id:"abcdg", name: "Some Put", method: "PUT", url: "https://httpbin.org/put"};
const deleteAct = { id:"abcdc", name: "Some Delete", method: "DELETE", url: "https://httpbin.org/delete"};
const defaultLib = { id:"lib1", name: "Library", actions: [getAct, postAct, putAct, deleteAct] };

let lib = [{...defaultLib, children: [{...defaultLib, id: "child1", name: "Child"}, {...defaultLib, id: "child2", name: "Child2"}]}];

