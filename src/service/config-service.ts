import { invoke } from "@tauri-apps/api";
import type { Config } from "../types";
import type { Action, LibraryNode } from "../pages/library/types";

export async function getConfig(): Promise<Config> {
  return await invoke("initialize");
}

export async function putAction(
  libId: string | null = null
): Promise<LibraryNode[]> {
  if (!libId) {
    return [makeLib(), ...lib];
  }
  return lib;
}

/*
 * temporary stubs
 */
function makeLib(): LibraryNode {
  return {
    id: "l" + Date.now().toString(),
    name: "newlib",
    actions: [
      {
        id: "a" + Date.now().toString(),
        name: "newact",
        method: "GET",
        url: "https://chiwawa.com",
        headers: [],
        query: [],
        path: [],
        body: "",
      },
    ],
  };
}

const getAct: Action = {
  id: "abcde",
  name: "Some Get",
  method: "GET",
  url: "https://httpbin.org/get",
  headers: [],
  query: [],
  path: [],
  body: "",
};
const postAct: Action = {
  id: "abcdf",
  name: "Some Post",
  method: "POST",
  url: "https://httpbin.org/post",
  headers: [],
  query: [],
  path: [],
  body: "",
};
const putAct: Action = {
  id: "abcdg",
  name: "Some Put",
  method: "PUT",
  url: "https://httpbin.org/put",
  headers: [],
  query: [],
  path: [],
  body: "",
};
const deleteAct: Action = {
  id: "abcdc",
  name: "Some Delete",
  method: "DELETE",
  url: "https://httpbin.org/delete",
  headers: [],
  query: [],
  path: [],
  body: "",
};
const defaultLib: LibraryNode = {
  id: "lib1",
  name: "Library",
  actions: [getAct, postAct, putAct, deleteAct],
};

let lib = [
  {
    ...defaultLib,
    children: [
      { ...defaultLib, id: "child1", name: "Child" },
      { ...defaultLib, id: "child2", name: "Child2" },
    ],
  },
];
