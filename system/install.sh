cd /
echo Make sure you run this as root!
echo Updating Debian
apt-get update &&
apt-get upgrade -y
apt-get install -y curl nano sudo git build-essential

echo Cloning game files
git clone https://github.com/mafia-rust/mafia.git

echo Installing Node
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
nvm install 20

echo Installing Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y
rustup toolchain install nightly --allow-downgrade --profile minimal --component clippy

echo Linking Services
ln -s /mafia/system/mafia-game-client.service /etc/systemd/system/mafia-game-client.service
ln -s /mafia/system/mafia-game-server.service /etc/systemd/system/mafia-game-server.service

echo Creating mafia user
adduser -disabled-password --gecos "" mafia
usermod -aG sudo mafia

echo Setting Permissions
chmod +x update.sh
chmod +x update-rootless.sh
chmod +x start-game-client.sh
chmod +x start-game-server.sh
chown -R mafia:mafia /mafia

echo Creating configuration files
mkdir system-config
cp client/src/resources/config.json system-config/client-config.json
cp server/resources/config.json system-config/server-config.json

echo Edit Client Configuration File
nano system-config/client-config.json

echo Edit Server Configuration File
nano system-config/server-config.json

echo Bootstrapping Mafia
runuser -l mafia -c './update-rootless.sh'

echo Reloading Daemons
systemctl daemon-reload

echo Enabling Mafia Services
systemctl enable mafia-game-client
systemctl enable mafia-game-server

echo Starting Mafia Services
systemctl start mafia-game-client
systemctl start mafia-game-server

echo Done!
cat next-steps.txt