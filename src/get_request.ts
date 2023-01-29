export const get_request = async (url: string): Promise<string> => {
  const url_request = await fetch(url)
  return await url_request.text()
}
