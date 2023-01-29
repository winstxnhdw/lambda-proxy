import { expect } from 'chai'
import { get_request } from '@/get_request'

describe('get_request', () => {
  it('It should return a valid body', () =>
    expect(get_request('https://nodeflair.com/api/v2/jobs?query=&page=1&sort_by=relevant')).to.be.a.string)
})
