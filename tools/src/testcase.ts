import fs = require('fs')
import cheerio = require('cheerio')

export function parseBodyAndSaveTestcases(body: string) {
  const $ = cheerio.load(body)

  const title = $('title').text()
  console.log(`* Title: ${title}`)

  const match = body.match(/userScreenName\s*=\s*"(.*)"/)
  if (match) {
    console.log(`* Login: ${match[1]}`)
  } else {
    console.log(`* Login: Guest access`)
  }

  $('#task-statement .part').each(function(this: any, i: Number) {
    const $el = $(this)
    const title = $el.find('h3').text()
    const match = title.match(/Sample\s+(Input|Output)\s+(\d+)/i)
    if (!match) return

    const type = match[1].toLowerCase()
    const number = match[2]
    const content = $el.find('pre').text().trim() + "\n"
    const filename = `${type}${number}`

    console.log(`* Write ${filename}`);

    fs.writeFileSync(filename, content)
  })
}