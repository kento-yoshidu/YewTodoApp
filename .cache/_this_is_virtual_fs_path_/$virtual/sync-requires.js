
// prefer default export if available
const preferDefault = m => (m && m.default) || m


exports.components = {
  "component---cache-dev-404-page-js": preferDefault(require("/home/kento/workspace/my-gatsby-site/.cache/dev-404-page.js")),
  "component---src-pages-404-js": preferDefault(require("/home/kento/workspace/my-gatsby-site/src/pages/404.js")),
  "component---src-pages-index-js": preferDefault(require("/home/kento/workspace/my-gatsby-site/src/pages/index.js"))
}

