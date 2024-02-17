
alacritty -e fish -c 'cd client/ && pnpm run dev; exec fish' &
alacritty -e fish -c 'cd mailer/ && pnpm run dev; exec fish' &
alacritty -e fish -c 'cd storage/ && pnpm run dev; exec fish' &

cd api/ && cargo watch -c -w src -x run