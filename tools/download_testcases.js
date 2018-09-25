const request = require('request')
const path = require('path')
const fs = require('fs')

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

const parseBody = body => {
  const $ = cheerio.load(body)
  $('#task-statement .part').each(function(i, e) {
    const title = $(this).find('h3').text()
    const match = title.match(/Sample\s+(Input|Output)\s+(\d+)/i)
    if (!match) return

    const type = match[1].toLowerCase()
    const number = match[2]
    const content = $(this).find('pre').text().trim() + "\n"
    const filename = `${type}${number}`
    console.log(`Write ${filename}`);
    fs.writeFileSync(filename, content, function (err) {
      if (err) {
          throw err;
      }
    });
  })
}

const handleUrl = url => {
  console.log(`Download testcases from ${url}.`)
  request(url, (error, response, body) => {
    if (error) {
      console.log(error)
    } else {
      parseBody(body)
    }
  })
}

const handleLocalFile = path => {
  const body = fs.readFileSync(path, 'utf8')
  parseBody(body)
}

const cheerio = require('cheerio')
const url = getUrl()

if (url.match(/^http/)) {
  handleUrl(url)
} else {
  handleLocalFile(url)
}
