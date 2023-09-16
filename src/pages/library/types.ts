export type Address = {
  method: string;
  url: string;
};

export type EnabledKvp<TKey, TVal> = {
  isEnabled: boolean,
  key: TKey,
  value: TVal
}

export type Action = Address & {
  id: string;
  name: string;
  headers?: EnabledKvp<string, string>[];
  parameters?: EnabledKvp<string, string>[];
};

export type LibraryNode = {
  id: string;
  name: string;
  actions?: Action[];
  children?: LibraryNode[];
};

export type ChiResponse = {
  body: string;
  headers?: [string, string][];
  status: number;
};
