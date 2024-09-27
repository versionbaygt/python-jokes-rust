import click
from _jokes import _jokes

@click.command()
@click.option('-l', '--language', default='en', help='Language for the joke')
def main(language):
    if language == 'en':
        print(_jokes.get_random_joke())
    else:
        print(f"No jokes available in language '{language}'.")

if __name__ == '__main__':
    main()
