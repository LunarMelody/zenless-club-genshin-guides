import type { PathLike } from "fs";
import { constants } from "fs";
import { access } from "fs/promises";

export const fileExists = async (file: PathLike) => {
  try {
    await access(file, constants.F_OK);
    return true;
  } catch {
    return false;
  }
};
