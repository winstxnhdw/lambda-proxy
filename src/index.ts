import type { Handler, APIGatewayProxyResultV2 } from 'aws-lambda'
import { get_request } from '@/get_request'

type LambdaFunctionURLEvent = {
  version: string
  routeKey: string
  rawPath: string
  rawQueryString: string
  headers: {
    host: string
  }
  queryStringParameters?: {
    [key: string]: unknown
  }
  requestContext: {
    accountId: string
    apiId: string
    domainName: string
    domainPrefix: string
    http: {
      method: 'GET' | 'POST'
      path: string
      protocol: string
      sourceIp: string
      userAgent: string | null
    }
    requestId: string
    routeKey: string
    stage: string
    time: string
    timeEpoch: number
  }
  body?: string
  isBase64Encoded: false
}

export const handler: Handler = async (event: LambdaFunctionURLEvent): Promise<APIGatewayProxyResultV2> => {
  if (event.requestContext.http.method !== 'GET') return { statusCode: 405, body: 'Method not allowed.' }
  if (!event.queryStringParameters?.['url']) return { statusCode: 400, body: 'Missing URL.' }

  const decoded_url = decodeURIComponent(event.queryStringParameters['url'] as string)
  const url_response = await get_request(decoded_url)
  console.log(`URL: ${decoded_url}\n\nResponse:\n${url_response}`)

  return {
    statusCode: 200,
    body: url_response
  }
}
