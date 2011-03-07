from distutils.core import setup

context = {}
execfile('sala', context, context)

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
    scripts=['sala'],
    classifiers=[
        'Programming Language :: Python',
        'Programming Language :: Python :: 2.5',
        'Programming Language :: Python :: 2.6',
        'Programming Language :: Python :: 2.7',
        'License :: OSI Approved :: MIT License',
        'Environment :: Console',
        'Topic :: Utilities',
    ],
)
