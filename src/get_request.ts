export const get_request = async (url: string): Promise<string> => {
  const request = await fetch(url)
  return request.text()
}
