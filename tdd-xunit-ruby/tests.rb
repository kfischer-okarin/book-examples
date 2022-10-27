require_relative 'xunit'

class WasRun < TestCase
  def initialize(name)
    super
    @was_run = false
  end

  def was_run?
    @was_run
  end

  def test_method
    @was_run = true
  end
end

test = WasRun.new('test_method')
puts test.was_run?
test.run
puts test.was_run?
