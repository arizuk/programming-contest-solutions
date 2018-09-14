const rp = require('request-promise')
const path = require('path')
const fs = require('fs')
const cheerio = require('cheerio')

const crawl = async () => {
  for(let i = 0; i < 30; i++) {
    const url = "http://atcoder.jp/ranking?p=" + (i + 1)
    const body = await rp(url).catch(err => console.log(err))
    parseBody(body)
  }
}

const parseBody = (body) => {
  const $ = cheerio.load(body)
  $('table.table tbody tr').each(function(i, e) {
    const td = $(this).find('td')
    const rank = td.eq(0).text()
    const img = td.eq(1).find('img')
    const imgPaths = img.attr('src').split("/")
    const country = imgPaths[imgPaths.length-1].replace('.png', '')
    const username = td.eq(1).find('a span').text()
    const rating = td.eq(2).text()
    console.log([rank, country, username, rating].join("\t"))
  })
}

crawl()