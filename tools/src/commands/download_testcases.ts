import {Command, flags} from '@oclif/command'
import {parseBodyAndSaveTestcases} from '../testcase'
import {restoreCookieJar} from '../login'
import got = require('got')
import fs = require('fs')

function handleLocalFile(path: string) {
  const body = fs.readFileSync(path, 'utf8')
  parseBodyAndSaveTestcases(body)
}

async function handleUrl(url: string) {
  console.log(`Download testcases from ${url}.`)
  const cookieJar = restoreCookieJar()
  const res = await got(url, {cookieJar});
  if (res.statusCode == 200) {
    parseBodyAndSaveTestcases(res.body)
  } else {
    throw new Error(`The server returns ${res.statusCode}`)
  }
}

export default class Login extends Command {
  static description = 'Parse and save testcases from given problem resource'

  static examples = [
    `$ atcoder download_testcases https://beta.atcoder.jp/contests/agc020/tasks/agc020_a`
  ]

  static args = [
    {
      name: 'url_or_file',
      required: true,
      description: "A problem page url or a local file path"
    }
  ]

  async run() {
    const {args, flags} = this.parse(Login)
    const urlOrFile = args.url_or_file
    if (urlOrFile.match(/^http/)) {
      await handleUrl(urlOrFile)
    } else {
      handleLocalFile(urlOrFile)
    }
  }
}