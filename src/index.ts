import type {
  Handler,
  APIGatewayProxyResultV2,
  APIGatewayProxyEventV2WithRequestContext,
  APIGatewayEventRequestContextV2
} from 'aws-lambda'
import { get_request } from '@/get_request'

const invalid_response = (body: string, statusCode = 400) => {
  return {
    statusCode: statusCode,
    body: body
  } as APIGatewayProxyResultV2
}

export const handler: Handler = async (
  event: APIGatewayProxyEventV2WithRequestContext<APIGatewayEventRequestContextV2>
): Promise<APIGatewayProxyResultV2> => {
  if (event.body === undefined) return invalid_response('Missing body.')

  const request = JSON.parse(event.body) as { endpoints?: string[] }
  if (request.endpoints === undefined) return invalid_response('Missing endpoints field.')
  if (request.endpoints.length === 0) return invalid_response('No endpoint(s) provided.')

  const responses = await Promise.all(request.endpoints.map(async (endpoint) => await get_request(endpoint))).catch(
    (error) => {
      return invalid_response(error.message, 500)
    }
  )

  return {
    statusCode: 200,
    body: JSON.stringify(responses)
  }
}
