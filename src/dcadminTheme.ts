// You can also use the generator at https://skeleton.dev/docs/generator to create these values for you
import type { CustomThemeConfig } from '@skeletonlabs/tw-plugin';
export const dcadminTheme: CustomThemeConfig = {
	name: 'dcadminTheme',
	properties: {
		// =~= Theme Properties =~=
		"--theme-font-family-base": `ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace`,
		"--theme-font-family-heading": `Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'`,
		"--theme-font-color-base": "0 0 0",
		"--theme-font-color-dark": "255 255 255",
		"--theme-rounded-base": "8px",
		"--theme-rounded-container": "8px",
		"--theme-border-base": "2px",
		// =~= Theme On-X Colors =~=
		"--on-primary": "0 0 0",
		"--on-secondary": "255 255 255",
		"--on-tertiary": "0 0 0",
		"--on-success": "0 0 0",
		"--on-warning": "0 0 0",
		"--on-error": "255 255 255",
		"--on-surface": "255 255 255",
		// =~= Theme Colors  =~=
		// primary | #d53b2a 
		"--color-primary-50": "249 226 223", // #f9e2df
		"--color-primary-100": "247 216 212", // #f7d8d4
		"--color-primary-200": "245 206 202", // #f5ceca
		"--color-primary-300": "238 177 170", // #eeb1aa
		"--color-primary-400": "226 118 106", // #e2766a
		"--color-primary-500": "213 59 42", // #d53b2a
		"--color-primary-600": "192 53 38", // #c03526
		"--color-primary-700": "160 44 32", // #a02c20
		"--color-primary-800": "128 35 25", // #802319
		"--color-primary-900": "104 29 21", // #681d15
		// secondary | #692aa7 
		"--color-secondary-50": "233 223 242", // #e9dff2
		"--color-secondary-100": "225 212 237", // #e1d4ed
		"--color-secondary-200": "218 202 233", // #dacae9
		"--color-secondary-300": "195 170 220", // #c3aadc
		"--color-secondary-400": "150 106 193", // #966ac1
		"--color-secondary-500": "105 42 167", // #692aa7
		"--color-secondary-600": "95 38 150", // #5f2696
		"--color-secondary-700": "79 32 125", // #4f207d
		"--color-secondary-800": "63 25 100", // #3f1964
		"--color-secondary-900": "51 21 82", // #331552
		// tertiary | #ca9fcb 
		"--color-tertiary-50": "247 241 247", // #f7f1f7
		"--color-tertiary-100": "244 236 245", // #f4ecf5
		"--color-tertiary-200": "242 231 242", // #f2e7f2
		"--color-tertiary-300": "234 217 234", // #ead9ea
		"--color-tertiary-400": "218 188 219", // #dabcdb
		"--color-tertiary-500": "202 159 203", // #ca9fcb
		"--color-tertiary-600": "182 143 183", // #b68fb7
		"--color-tertiary-700": "152 119 152", // #987798
		"--color-tertiary-800": "121 95 122", // #795f7a
		"--color-tertiary-900": "99 78 99", // #634e63
		// success | #84cc16 
		"--color-success-50": "237 247 220", // #edf7dc
		"--color-success-100": "230 245 208", // #e6f5d0
		"--color-success-200": "224 242 197", // #e0f2c5
		"--color-success-300": "206 235 162", // #ceeba2
		"--color-success-400": "169 219 92", // #a9db5c
		"--color-success-500": "132 204 22", // #84cc16
		"--color-success-600": "119 184 20", // #77b814
		"--color-success-700": "99 153 17", // #639911
		"--color-success-800": "79 122 13", // #4f7a0d
		"--color-success-900": "65 100 11", // #41640b
		// warning | #EAB308 
		"--color-warning-50": "252 244 218", // #fcf4da
		"--color-warning-100": "251 240 206", // #fbf0ce
		"--color-warning-200": "250 236 193", // #faecc1
		"--color-warning-300": "247 225 156", // #f7e19c
		"--color-warning-400": "240 202 82", // #f0ca52
		"--color-warning-500": "234 179 8", // #EAB308
		"--color-warning-600": "211 161 7", // #d3a107
		"--color-warning-700": "176 134 6", // #b08606
		"--color-warning-800": "140 107 5", // #8c6b05
		"--color-warning-900": "115 88 4", // #735804
		// error | #ff0000 
		"--color-error-50": "255 217 217", // #ffd9d9
		"--color-error-100": "255 204 204", // #ffcccc
		"--color-error-200": "255 191 191", // #ffbfbf
		"--color-error-300": "255 153 153", // #ff9999
		"--color-error-400": "255 77 77", // #ff4d4d
		"--color-error-500": "255 0 0", // #ff0000
		"--color-error-600": "230 0 0", // #e60000
		"--color-error-700": "191 0 0", // #bf0000
		"--color-error-800": "153 0 0", // #990000
		"--color-error-900": "125 0 0", // #7d0000
		// surface | #282a2f 
		"--color-surface-50": "223 223 224", // #dfdfe0
		"--color-surface-100": "212 212 213", // #d4d4d5
		"--color-surface-200": "201 202 203", // #c9cacb
		"--color-surface-300": "169 170 172", // #a9aaac
		"--color-surface-400": "105 106 109", // #696a6d
		"--color-surface-500": "40 42 47", // #282a2f
		"--color-surface-600": "36 38 42", // #24262a
		"--color-surface-700": "30 32 35", // #1e2023
		"--color-surface-800": "24 25 28", // #18191c
		"--color-surface-900": "20 21 23", // #141517
		
	}
}