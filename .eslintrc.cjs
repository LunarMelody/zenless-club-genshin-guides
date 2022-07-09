module.exports = {
  root: true,
  parser: `@typescript-eslint/parser`,
  plugins: [`@typescript-eslint`],
  extends: [
    `plugin:@typescript-eslint/recommended`,
    `plugin:prettier/recommended`,
  ],
  env: {
    node: true,
    es6: true,
  },
  rules: {
    "@typescript-eslint/consistent-type-imports": 2,
    "@typescript-eslint/require-await": `off`,
    "@typescript-eslint/no-unsafe-assignment": `off`,
    "@typescript-eslint/restrict-template-expressions": `off`,
    "@next/next/no-img-element": `off`,
    "react-hooks/exhaustive-deps": `off`,
  },
  settings: {
    "import/resolver": {
      typescript: {
        alwaysTryTypes: true,
        project: `./tsconfig.json`,
      },
    },
  },
  ignorePatterns: [`node_modules/`, `build/`, `dist/`, `out/`],
};
