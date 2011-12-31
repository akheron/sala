from distutils.core import setup

# Read metadata (__version__ etc.) from sala/__init__.py
context = {}
with open('sala/__init__.py') as fobj:
    lines = []
    save = False
    for line in fobj:
        if not save and line.strip() == '## START OF METADATA ##':
            save = True
        elif save and line.strip() == '## END OF METADATA ##':
            break

        if save:
            lines.append(line)

    exec(''.join(lines), context)

setup(
    name='sala',
    version=context['__version__'],
    author='Petri Lehtinen',
    author_email='petri@digip.org',
    description='Encrypted plaintext password store',
    long_description=''.join([
        open('README.rst').read(),
        '\n\n',
        open('CHANGES').read()
    ]),
    license='MIT',
    packages=['sala'],
    scripts=['bin/sala'],
    classifiers=[
        'Programming Language :: Python',
        'Programming Language :: Python :: 2.6',
        'Programming Language :: Python :: 2.7',
        'Programming Language :: Python :: 3',
        'Programming Language :: Python :: 3.2',
        'License :: OSI Approved :: MIT License',
        'Environment :: Console',
        'Topic :: Utilities',
    ],
)
