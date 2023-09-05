export type Address = {
  method: string;
  url: string;
};

export type Headers = Map<string, string>;

export type Activity = Address & {
  name: string
  headers?: Headers 
};

export type LibraryNode = {
  name: string;
  activities?: Activity[];
  subs?: LibraryNode[];
};
