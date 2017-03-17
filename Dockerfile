FROM alpine:3.5.1

# Install Dependencies
RUN apk update \
 && apk add git ca-certificates gcc make rust linux-headers ffmpeg

# Add project source
ADD . /usr/src/DiscordRsBot
WORKDIR /usr/src/DiscordRsBot

# Create volume for mapping the config
VOLUME /usr/src/DiscordRsBot/config

CMD ./bot
