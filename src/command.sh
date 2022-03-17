# this is ridiculous - how many programs do you need to archive a forum, fast?
output_dir="$HOME/fusornet"
RUST_BACKTRACE=1 cargo run -- http://fusor.net --jobs 16 --verbose --output $output_dir --exclude-download '(?:view=print)|(?:viewtopic\.php.*p=)|(?:posting|search|ucp|viewonline|memberlist)\.php|hilit=' --tries 50 --visit-filter-is-download-filter | tee /tmp/slog
cd $output_dir
wget --page-requisites fusor.net/board/ --accept '*.css*' --output-document fusor.net/board/styles/prosilver/theme/stylesheet.css__q*
