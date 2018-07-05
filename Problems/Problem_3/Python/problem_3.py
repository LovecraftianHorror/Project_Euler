#!/usr/bin/env python3


def largest_prime_factor(number):
    assert(number > 1)
    index = 3

    while not number % 2:
        number /= 2

    while True:
        # Reduce given number
        while not number % index:
            number /= index

        # Number is fully reduced, we are done
        if number == 1:
            break

        # Get next prime
        index += 2

    return index


if __name__ == '__main__':
    number = 600851475143
    prime_factor = largest_prime_factor(number)
    print(f'The largest prime factor of {number} is {prime_factor}')


