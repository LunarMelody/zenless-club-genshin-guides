import { z } from "zod";
import { Meta } from "./meta";

export const Guide = z.object({
  meta: Meta,
  html: z.string(),
});
export type GuideType = z.infer<typeof Guide>;
