# lambda-proxy

[![main.yml](https://github.com/winstxnhdw/lambda-proxy/actions/workflows/main.yml/badge.svg)](https://github.com/winstxnhdw/lambda-proxy/actions/workflows/main.yml)

My personal proxy with AWS Lambda. You may fork this repository and follow the instructions in [Setup](#setup).

## Purpose

I wanted to use the same API request that an organisation uses for a clone of their website that I was building. Their website had CORS enabled and my website was not able to retrieve the API response. As the name suggests, `lambda-proxy` is a lightweight proxy, primarily for completing a fetch request on a Node.js instance, bypassing the earlier mentioned CORS issue.

## Setup

The following instructions uses the [GitHub](https://cli.github.com/) and [AWS](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) CLI to setup this proxy. You only need to follow these steps once.

### Role

Create a AWS role.

```bash
aws iam create-role --role-name <role-name> --assume-role-policy-document '{"Version": "2012-10-17","Statement": [{ "Effect": "Allow", "Principal": {"Service": "lambda.amazonaws.com"}, "Action": "sts:AssumeRole"}]}'
```

Attach the `AWSLambdaBasicExecutionRole` policy to your created role.

```bash
aws iam attach-role-policy --role-name <role-name> --policy-arn arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole
```

### Function

Get your AWS account ID.

```bash
aws sts get-caller-identity --query "Account" --output text
```

Create a AWS Lambda function with the `Node.js 18.x` runtime and `x86_64` architecture. See why [here](https://www.amanox.ch/en/awslambda/).

```bash
aws lambda create-function --function-name lambda-proxy --handler index.handler --runtime nodejs18.x --role arn:aws:iam::<account-id>:role/<role-name>
```

### Additional Configuration

Add a Lambda function URL and keep the output URL safe. Learn more [here](https://docs.aws.amazon.com/cli/latest/reference/lambda/create-function-url-config.html).

> I prefer using Lambda Function URLs for speed, but you can use an API Gateway if you know what you are doing.

```bash
aws create-function-url-config --function-name lambda-proxy --auth-type NONE --cors { \
  "AllowMethods": ["GET"], \
  "AllowOrigins": ["*"], \ # You should be using specific origins instead
}
```

### Workflow

Add the `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY` secrets to the repository.

```bash
gh secret set AWS_ACCESS_KEY_ID --repos lambda-proxy --body <AWS_ACCESS_KEY_ID>
gh secret set AWS_SECRET_ACCESS_KEY --repos lambda-proxy --body <AWS_SECRET_ACCESS_KEY>
```

Start the CI/CD workflow.

```bash
gh workflow run main.yml
```

## Usage

```ts
import { config } from '@/config'

const getRequestWithProxy = async (url: string) => {
  const request = await fetch(
    `${config.VITE_PROXY_ENDPOINT}?${new URLSearchParams({
      url: encodeURIComponent(url),
    })}`
  )

  return request.json()
}

const response = await getRequestWithProxy('https://account.battleon.com/charpage/details?id=53251829')
console.log(response)
```
