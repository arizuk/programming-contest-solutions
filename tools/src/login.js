const request = require('request')
const cheerio = require('cheerio')
const fs = require('fs')
const path = require("path")
const { CookieJar } = require('tough-cookie')

const url = "https://beta.atcoder.jp/login"
const cookieStorePath = path.join(process.env.HOME, ".atcoder.json")

const setCookies = (jar, response) => {
  const cookies = response.headers['set-cookie'] || []
  cookies.forEach(cookieString => {
    const cookie = request.cookie(cookieString)
    jar.setCookie(cookie, url)
  })
}

const parseCsrfToken = (body) => {
  const $ = cheerio.load(body)
  return $('input[name="csrf_token"]').val()
}

const getLoginPage = () => {
  return new Promise((resolve, reject) => {
    request.get(url, (error, response, body) => resolve({ error, response, body }))
  })
}

const login = async () => {
  const jar = request.jar()

  const { error, body, response } = await getLoginPage()

  setCookies(jar, response)
  const csrfToken = parseCsrfToken(body)

  const formData = {
    username: process.env.ATCODER_USER,
    password: process.env.ATCODER_PASSWORD,
    csrf_token: csrfToken
  }

  request.post({ url, jar, formData }, (error, response, body) => {
    if (response.statusCode == 302 && response.headers.location === '/') {
      setCookies(jar, response)
      console.log(`Login atcoder successfully`);
      const json = JSON.stringify(jar._jar.toJSON());
      fs.writeFileSync(cookieStorePath, json, (err) => {
        if (err) throw(err)
      })
    } else {
      console.log(`Login failed`);
    }
  })
}

const restoreCookieJar = () => {
  const requestJar = request.jar()
  const json = JSON.parse(fs.readFileSync(cookieStorePath, 'utf8'))
  const jar = CookieJar.fromJSON(json)
  requestJar._jar = jar
  return requestJar
}

module.exports = {
  login,
  restoreCookieJar
}