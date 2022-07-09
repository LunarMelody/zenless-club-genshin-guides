import { z } from "zod";

const GuideType = z.union([z.literal("character"), z.literal("general")]);

const MetaRest = z.record(z.union([z.string(), z.number()]));

export const MetaRequired = z.object({
  id: z.string(),
  type: GuideType,
});

export const Meta = MetaRequired.and(MetaRest);
export type MetaType = z.infer<typeof Meta>;
