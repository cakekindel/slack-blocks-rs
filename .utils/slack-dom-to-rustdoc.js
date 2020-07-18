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
        commaRetIndent,
        periodRetRet,
        stripLinks,
        moveLinkDefs,
        docComment,
    ];

    // Flow stdin through each transformer
    const output = transformers.reduce((c, t) => t(c), contents);

    process.stdout.write('\n\n\n');
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

// Turn anchors into MD reference-style links
const stripLinks = helpers.replace(/<a href\="(.*?)">(.*?)<\/a>/gi, '[$2 🔗]%%[$2 🔗]: https://api.slack.com$1\n%%');

// break lists up with newline + indent
const commaRetIndent = helpers.replace(/,/gi, ',\n    ');

// Separate sentences with double newline
const periodRetRet = helpers.replace(/\./gi, '.\n\n');

// Move MD link definitions to the bottom
const moveLinkDefs = c => c.split(/%%/g).sort((a, b) => helpers.isLink(a) ? 1 : helpers.isLink(b) ? -1 : 0).join('');

// Prepend each line with ///
const docComment = helpers.eachLine(c => '/// ' + c);

