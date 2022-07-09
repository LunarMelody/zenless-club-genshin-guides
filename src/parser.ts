import { parse as parseYaml } from "yaml";
import { unified } from "unified";
import remarkParse from "remark-parse";
import remarkFrontmatter from "remark-frontmatter";
import remarkGfm from "remark-gfm";
import remarkRehype from "remark-rehype";
import rehypeStringify from "rehype-stringify";
import rehypeSlug from "rehype-slug";
import rehypeSanitize from "rehype-sanitize";
import remarkStringify from "remark-stringify";
import rehypePresetMinify from "rehype-preset-minify";

// only god knows the hell is going on here
// honestly try not to get lost in all the
// remark, unified, rehype and etc. repos

const parser = unified()
  .use(remarkParse)
  .use(remarkStringify)
  .use(remarkFrontmatter, ["yaml"])
  .use(() => (tree, file) => {
    const meta = tree.children.find((c) => c.type === "yaml");
    if (meta != null) {
      if ("value" in meta) {
        file.data.meta = parseYaml(meta.value);
      }
    }
  })
  .use(remarkGfm)
  .use(remarkRehype)
  .use(rehypeSanitize)
  .use(rehypeSlug)
  .use(rehypePresetMinify)
  .use(rehypeStringify);

export const parseMarkdown = async (content: Buffer) => {
  return await parser.process(content);
};
