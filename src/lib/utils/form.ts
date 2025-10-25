export function createInputUpdater(updateFn: (value: string) => unknown) {
  return function updateField(
    e: InputEvent & { currentTarget: HTMLInputElement }
  ) {
    updateFn(e.currentTarget.value);
  };
}
