import {Command, flags} from '@oclif/command'
import {login} from '../login'

export default class Login extends Command {
  static description = 'login to atcoder.jp'

  static examples = [
    `$ atcoder login`
  ]

  static args = [{name: 'url'}]

  async run() {
    const {args, flags} = this.parse(Login)

    const url = args.url || 'https://beta.atcoder.jp/login'
    this.log(`Login to ${url}`)
    await login(url)
  }
}