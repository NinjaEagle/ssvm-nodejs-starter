import { AzureFunction, Context, HttpRequest } from '@azure/functions'
const { generate } = require('./wasm_loader')

const func: AzureFunction = async function (
	context: Context,
	req: HttpRequest
): Promise<void> {
	try {
		const name: string = await generate()
		const [adjective, noun] = name.split(' ')
		console.log('Generated random name:', adjective, noun)

		context.res = {
			body: {
				adjective,
				noun,
			},
		}
	} catch (error) {
		context.res = {
			body: {
				error,
			},
		}
	}
}

export default func
