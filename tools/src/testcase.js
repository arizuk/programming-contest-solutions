const fs = require('fs')
const cheerio = require('cheerio')

const parseBodyAndSaveTestcases = body => {
  const $ = cheerio.load(body)
  const match = body.match(/userScreenName\s*=\s*"(.*)"/)
  if (match) {
    console.log(`* Logged in as ${match[1]}`)
  } else {
    console.log(`* Guest access`)
  }

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

module.exports = {
  parseBodyAndSaveTestcases
}