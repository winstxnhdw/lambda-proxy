import type {
  Handler,
  APIGatewayProxyResultV2,
  APIGatewayProxyEventV2WithRequestContext,
  APIGatewayEventRequestContextV2
} from 'aws-lambda'
import { get_request } from '@/get_request'

export const handler: Handler = async (
  event: APIGatewayProxyEventV2WithRequestContext<APIGatewayEventRequestContextV2>
): Promise<APIGatewayProxyResultV2> => {
  if (event.requestContext.http.method !== 'GET') return { statusCode: 405, body: 'Method not allowed.' }
  if (!event.queryStringParameters?.['url']) return { statusCode: 400, body: 'Missing URL.' }

  const decoded_url = decodeURIComponent(event.queryStringParameters['url'] as string)
  const url_response = await get_request(decoded_url)

  return {
    statusCode: 200,
    body: url_response
  }
}
