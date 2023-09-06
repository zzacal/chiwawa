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
  name: string
  headers?: EnabledKvp<string, string>[]
  parameters?: EnabledKvp<string, string>[]
};

export type LibraryNode = {
  name: string;
  actions?: Action[];
  children?: LibraryNode[];
};
