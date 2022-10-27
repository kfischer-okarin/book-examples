require_relative 'xunit'

class WasRun < TestCase
  attr_reader :log

  def was_run?
    @was_run
  end

  def was_set_up?
    @was_set_up
  end

  def set_up
    @was_run = false
    @was_set_up = true
    @log = 'set_up '
  end

  def test_method
    @was_run = true
  end
end

class TestCaseTest < TestCase
  def set_up
    @test = WasRun.new('test_method')
  end

  def test_running
    assert! !@test.was_run?
    @test.run
    assert! @test.was_run?
  end

  def test_set_up
    @test.run
    assert! @test.log == 'set_up '
  end
end

TestCaseTest.new('test_running').run
TestCaseTest.new('test_set_up').run
