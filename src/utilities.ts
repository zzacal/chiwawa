
export const onAffirmativeKey = (handler: () => void) => (event: KeyboardEvent) => {
  if (event.key === 'Enter' || event.key === ' ') {
    return handler();
  }
}

export const onEnterKey =
  (handler: () => void) => (event: KeyboardEvent) => {
    if (event.key === "Enter") {
      return handler();
    }
  };
