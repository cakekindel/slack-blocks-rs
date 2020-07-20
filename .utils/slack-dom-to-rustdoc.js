process.stdin.setEncoding('utf8');

let contents = '';
process.stdin.on('readable', () => {
    let chunk = process.stdin.read();
    while (chunk) {
        contents += chunk;
        chunk = process.stdin.read();
    }
});

process.stdin.on('end', () => {
    // array of String -> String map
    // functions that the stdin will
    // be piped through
    const transformers = [
        periodRet,
        qualifyRootLinks,
        qualifyAnchors,
        escapeLinks,
        moveLinkDefs,
        escapeCode,
        escapeStrong,
        docComment,
    ];

    // Flow stdin through each transformer
    const output = transformers.reduce((c, t) => t(c), contents);

    process.stdout.write(output);
});

const helpers = {
    isLink: str => (/\[.*?\]\:/i).test(str),
    replace: (pat, replace) => contents => {
        const output = contents.replace(pat, replace);
        return output;
    },
    eachLine: map => contents => contents.split('\n')
                                         .map(map)
                                         .join('\n'),
}

// # Transformers

// <a src="/link"> -> <a src="https://api.slack.com/link">
const qualifyRootLinks = helpers.replace(
  /<a href\="\/(.*?)">/gi,
  '<a href\="https://api.slack.com/$1">'
);

// <a src="#anchor"> -> <a src="CURRENT_PAGE#anchor">
const qualifyAnchors = helpers.replace(
  /<a href\="(.*?)">/gi,
  `<a href\="CURRENT_PAGE#$1">`
);

// <a src="/link">Text</a> -> [Text 🔗] [Text 🔗]: https://api.slack.com/link
const escapeLinks = helpers.replace(
  /<a href\="(.*?)">(.*?)<\/a>/gi,
  '[$2 🔗]%%[$2 🔗]: https://api.slack.com$1\n%%'
);

// <code>blah</code> -> `blah`
const escapeCode = helpers.replace(/<code>(.*?)<\/code>/gi, '`$1`');

// <strong>blah</strong> -> **blah**
const escapeStrong = helpers.replace(/<strong>(.*?)<\/strong>/gi, '**$1**');

// Separate sentences with newline
const periodRet = helpers.replace(/\./gi, '.\n');

// Move MD link definitions to the bottom
const moveLinkDefs = c => c.split(/%%/g).sort((a, b) => helpers.isLink(a) ? 1 : helpers.isLink(b) ? -1 : 0).join('');

// Prepend each line with ///
const docComment = helpers.eachLine(c => '/// ' + c);

