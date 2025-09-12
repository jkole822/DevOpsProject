export function getCookie(key: string) {
  const keyEQ = key + '='
  const ca = document.cookie.split(';')
  for (let i = 0; i < ca.length; i++) {
    let c = ca[i]
    while (c.charAt(0) === ' ') c = c.substring(1, c.length)
    if (c.indexOf(keyEQ) === 0) return c.substring(keyEQ.length, c.length)
  }
  return null
}

// Example Cookie with Expiration & Path:
// document.cookie = "username=John Doe; user_id=123; expires=Thu, 18 Dec 2025 12:00:00 UTC; path=/";
// ca = ["username=John Doe", " user_id=123", " expires=Thu, 18 Dec 2025 12:00:00 UTC", " path=/"]
// c(i = 0) = "username=John Doe"
// "username=John Doe".indexOf("user_id") !== 0
// Go to next iteration
// c(i = 1) = " user_id=123"
// c = " user_id=123".substring(1, 11)
// c = "user_id=123"
// return "user_id=123".substring("user_id=".length, "user_id=123".length)
// return "user_id=123".substring(8, 11)
// return "123"

export function deleteCookie(key: string, path = '/', domain = '') {
  document.cookie =
    `${key}=; expires=Thu, 01 Jan 1970 00:00:01 GMT; path=${path}` +
    (domain ? '; domain=' + domain : '')
}

export function setCookie(
  key: string,
  value: string,
  expires = new Date(new Date().setDate(new Date().getDate() + 30)).toUTCString(),
  path = '/',
  domain = '',
) {
  document.cookie =
    `${key}=${value}; expires=${expires}; path=${path}` + (domain ? '; domain=' + domain : '')
}
