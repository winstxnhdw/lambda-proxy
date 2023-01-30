# lambda-proxy

[![main.yml](https://github.com/winstxnhdw/lambda-proxy/actions/workflows/main.yml/badge.svg)](https://github.com/winstxnhdw/lambda-proxy/actions/workflows/main.yml)

My personal proxy with AWS Lambda.

## Usage

```ts
import { config } from '@/config'

const getRequestWithProxy = async (url: string) => {
  const request = await fetch(
    `${config.VITE_PROXY_ENDPOINT}?${new URLSearchParams({
      url: url
    })}`
  )

  return request.json()
}

const response = await getRequestWithProxy('https://account.battleon.com/charpage/details?id=53251829')
console.log(response)
```
