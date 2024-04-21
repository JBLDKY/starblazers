import type { CustomThemeConfig } from '@skeletonlabs/tw-plugin';

export const sbTheme: CustomThemeConfig = {
	name: 'sb-theme',
	properties: {
		// =~= Theme Properties =~=
		'--theme-font-family-base': `system-ui`,
		'--theme-font-family-heading': `system-ui`,
		'--theme-font-color-base': '0 0 0',
		'--theme-font-color-dark': '255 255 255',
		'--theme-rounded-base': '9999px',
		'--theme-rounded-container': '8px',
		'--theme-border-base': '1px',
		// =~= Theme On-X Colors =~=
		'--on-primary': '0 0 0',
		'--on-secondary': '255 255 255',
		'--on-tertiary': '0 0 0',
		'--on-success': '0 0 0',
		'--on-warning': '0 0 0',
		'--on-error': '255 255 255',
		'--on-surface': '255 255 255',
		// =~= Theme Colors  =~=
		// primary | #4a2eca
		'--color-primary-50': '228 224 247', // #e4e0f7
		'--color-primary-100': '219 213 244', // #dbd5f4
		'--color-primary-200': '210 203 242', // #d2cbf2
		'--color-primary-300': '183 171 234', // #b7abea
		'--color-primary-400': '128 109 218', // #806dda
		'--color-primary-500': '74 46 202', // #4a2eca
		'--color-primary-600': '67 41 182', // #4329b6
		'--color-primary-700': '56 35 152', // #382398
		'--color-primary-800': '44 28 121', // #2c1c79
		'--color-primary-900': '36 23 99', // #241763
		// secondary | #db3cf0
		'--color-secondary-50': '250 226 253', // #fae2fd
		'--color-secondary-100': '248 216 252', // #f8d8fc
		'--color-secondary-200': '246 206 251', // #f6cefb
		'--color-secondary-300': '241 177 249', // #f1b1f9
		'--color-secondary-400': '230 119 245', // #e677f5
		'--color-secondary-500': '219 60 240', // #db3cf0
		'--color-secondary-600': '197 54 216', // #c536d8
		'--color-secondary-700': '164 45 180', // #a42db4
		'--color-secondary-800': '131 36 144', // #832490
		'--color-secondary-900': '107 29 118', // #6b1d76
		// tertiary | #b5e6ff
		'--color-tertiary-50': '244 251 255', // #f4fbff
		'--color-tertiary-100': '240 250 255', // #f0faff
		'--color-tertiary-200': '237 249 255', // #edf9ff
		'--color-tertiary-300': '225 245 255', // #e1f5ff
		'--color-tertiary-400': '203 238 255', // #cbeeff
		'--color-tertiary-500': '181 230 255', // #b5e6ff
		'--color-tertiary-600': '163 207 230', // #a3cfe6
		'--color-tertiary-700': '136 173 191', // #88adbf
		'--color-tertiary-800': '109 138 153', // #6d8a99
		'--color-tertiary-900': '89 113 125', // #59717d
		// success | #15F5BA
		'--color-success-50': '220 254 245', // #dcfef5
		'--color-success-100': '208 253 241', // #d0fdf1
		'--color-success-200': '197 253 238', // #c5fdee
		'--color-success-300': '161 251 227', // #a1fbe3
		'--color-success-400': '91 248 207', // #5bf8cf
		'--color-success-500': '21 245 186', // #15F5BA
		'--color-success-600': '19 221 167', // #13dda7
		'--color-success-700': '16 184 140', // #10b88c
		'--color-success-800': '13 147 112', // #0d9370
		'--color-success-900': '10 120 91', // #0a785b
		// warning | #EAB308
		'--color-warning-50': '252 244 218', // #fcf4da
		'--color-warning-100': '251 240 206', // #fbf0ce
		'--color-warning-200': '250 236 193', // #faecc1
		'--color-warning-300': '247 225 156', // #f7e19c
		'--color-warning-400': '240 202 82', // #f0ca52
		'--color-warning-500': '234 179 8', // #EAB308
		'--color-warning-600': '211 161 7', // #d3a107
		'--color-warning-700': '176 134 6', // #b08606
		'--color-warning-800': '140 107 5', // #8c6b05
		'--color-warning-900': '115 88 4', // #735804
		// error | #D41976
		'--color-error-50': '249 221 234', // #f9ddea
		'--color-error-100': '246 209 228', // #f6d1e4
		'--color-error-200': '244 198 221', // #f4c6dd
		'--color-error-300': '238 163 200', // #eea3c8
		'--color-error-400': '225 94 159', // #e15e9f
		'--color-error-500': '212 25 118', // #D41976
		'--color-error-600': '191 23 106', // #bf176a
		'--color-error-700': '159 19 89', // #9f1359
		'--color-error-800': '127 15 71', // #7f0f47
		'--color-error-900': '104 12 58', // #680c3a
		// surface | #221569
		'--color-surface-50': '222 220 233', // #dedce9
		'--color-surface-100': '211 208 225', // #d3d0e1
		'--color-surface-200': '200 197 218', // #c8c5da
		'--color-surface-300': '167 161 195', // #a7a1c3
		'--color-surface-400': '100 91 150', // #645b96
		'--color-surface-500': '34 21 105', // #221569
		'--color-surface-600': '31 19 95', // #1f135f
		'--color-surface-700': '26 16 79', // #1a104f
		'--color-surface-800': '20 13 63', // #140d3f
		'--color-surface-900': '17 10 51' // #110a33
	}
};
