require_relative 'xunit'

test = WasRun.new('test_method')
puts test.was_run?
test.run
puts test.was_run?
