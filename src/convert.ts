import { globby } from "globby";
import fs from "fs-extra";
import { Command } from "commander";
import { join } from "path";
import { parseMarkdown } from "~/parser";
import type { MetaType } from "~/meta";
import { Meta, MetaRequired } from "~/meta";
import { parse as parsePath } from "path";
import { Guide } from "~/guide";

const program = new Command();

program
  .requiredOption("-i, --input <pattern>")
  .requiredOption("-o, --outdir <dir>")
  .parse(process.argv);

const options = program.opts();

const input = options.input as string;
const outDir = options.outdir as string;

// collect provded file paths from pattern with globby
const paths = await globby(join(input));
if (paths.length < 1) {
  console.log("Provided input directory is empty");
  process.exit(-1);
}

// clear resulting dir
await fs.emptyDir(outDir);

// store published guides meta in an array
const published = new Array<MetaType>();

paths.forEach(async (file) => {
  const content = await fs.readFile(file);
  const vfile = await parseMarkdown(content);

  // parse meta data to validate input data
  const meta = Meta.parse(vfile.data.meta);
  // parse meta again and store only required properties in published array
  published.push(MetaRequired.parse(meta));

  const result = JSON.stringify(Guide.parse({ meta, html: `${vfile}` }));
  const fileJson = parsePath(file).name + ".json";
  const resultPath = join(outDir, fileJson);

  await fs.ensureFile(resultPath);
  await fs.writeFile(resultPath, result, { encoding: "utf-8", flag: "w+" });
});

// store published guides array in json
const publishedArrayPath = join(outDir, "published.json");

await fs.ensureFile(publishedArrayPath);
await fs.writeJSON(publishedArrayPath, published, {
  encoding: "utf-8",
  flag: "w+",
});
