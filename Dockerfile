FROM rust

WORKDIR /usr/src/
RUN git clone https://github.com/frostblooded/pi_calc.git
WORKDIR /usr/src/pi_calc

CMD ["/usr/local/cargo/bin/cargo", "bench"]
