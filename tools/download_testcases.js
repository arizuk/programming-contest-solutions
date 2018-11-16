const request = require('request')
const path = require('path')
const fs = require('fs')
const cheerio = require('cheerio')
const { parseBodyAndSaveTestcases } = require('./src/testcase')
const { restoreCookieJar } = require('./src/login')

const getUrl = () => {
  if (process.argv.length > 2) {
    return process.argv[2];
  } else {
    const dirs = process.env.PWD.split(path.sep).reverse()
    const rank = dirs[0].toLowerCase()
    const contest = dirs[1].toUpperCase()
    return `https://beta.atcoder.jp/contests/${contest}/tasks/${contest}_${rank}`
  }
}

const handleUrl = url => {
  console.log(`Download testcases from ${url}.`)
  const jar = restoreCookieJar()
  request.get({ url, jar }, (error, response, body) => {
    if (error) {
      console.log(error)
    } else {
      parseBodyAndSaveTestcases(body)
    }
  })
}

const handleLocalFile = path => {
  const body = fs.readFileSync(path, 'utf8')
  parseBodyAndSaveTestcases(body)
}

const url = getUrl()

if (url.match(/^http/)) {
  handleUrl(url)
} else {
  handleLocalFile(url)
}
