// @ts-check
import eslintConfig from "@antfu/eslint-config";
import withNuxt from "./.nuxt/eslint.config.mjs";

export default withNuxt(
	eslintConfig(
		{
			typescript: true,
			vue: true,
			stylistic: {
				indent: "tab",
				quotes: "double"
			},
			ignores: [
				".github/**",
				"src-tauri/**",
				"**/*.md",
				"scripts/**"
			],
			rules: {
				curly: "off",
				"new-cap": "off",
				"no-console": "off",
				"no-new-func": "off",
				"style/semi": ["error", "always"],
				"style/indent": "off",
				"style/quote-props": "off",
				"style/comma-dangle": "off",
				"style/brace-style": "off",
				"style/arrow-parens": "off",
				"vue/block-order": ["error", {
					order: ["script", "template", "style"]
				}],
				"vue/script-indent": "off",
				"vue/singleline-html-element-content-newline": "off",
				"vue/multiline-html-element-content-newline": "off",
				"vue/max-attributes-per-line": "off",
				"vue/no-multiple-template-root": "off",
				"antfu/top-level-function": "off",
				"antfu/if-newline": "off",
				"node/prefer-global/process": "off",
				"unicorn/prefer-includes": "off"
			}
		},
		{
			files: ["**/*.vue"],
			rules: {
				"style/indent": "off"
			}
		}
	)
);
